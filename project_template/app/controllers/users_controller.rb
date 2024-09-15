require 'haml'
require 'bcrypt'
require 'logger'

class UsersController < ApplicationController
  # get '/users' do
  #   @users = User.all
  #   respond_to do
  #     json(except: [:password_hash]) { @users }
  #     html {haml: :users_index}
  #   end
  # end

  get '/signup' do
    @user = User.new
    haml :users_new
  end

  get '/user/profile/:id' do |id|
    user_service = UsersService.new(id:, current_user:)

    result = user_service.show_user

    error_response(result[:error]) do
      recover Err.not_found do
        flash[:error] = result[:message]
        redirect "/"
      end
      recover Err.unauthorized do
        flash[:error] = result[:message]
        redirect "/login"
      end
      recover :rest do
        @user = result[:user]
        flash[:error] = result[:message]
        haml :users_edit
      end
    end

    @user = result[:user]

    respond_to do
      json(except: [:password_hash]) { @user }
      html { haml :users_edit }
    end
  end

  post '/user' do
    user_service = UsersService.new(params:)
    result = user_service.create_user

    error_response result[:error] do
      recover :rest do
        @user = User.new(username: params[:username])
        flash[:error] = result[:message]
        haml :users_new
      end
    end

    flash[:notice] = 'Account created'
    redirect '/login' if result[:success]
  end

  put '/user/profile/:id' do |id|
    user_service = UsersService.new(id:, params:, current_user:)
    result = user_service.update_user

    error_response(result[:error]) do
      recover :rest do
        @user = User.new User.permitted(params)
        flash[:error] = result[:message]
        redirect "/user/profile/#{id}"
      end
    end

    flash[:notice] = 'User updated'
    redirect "/user/profile/#{id}"
  end

  delete '/user/:id' do |id|
    return access_denied unless user_has_access

    user = User.find(id:)
    user.destroy
    redirect '/'
  end
end
