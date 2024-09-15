module AuthHelpers
  def logged_in?
    !!session[:current_user]
  end

  def jwt_secret
    ENV['SESSION_SECRET']
  end

  def current_user
    return @current_user if @current_user
    token = request.cookies['jwt']
    if token.nil?
      return nil
    end
    begin
      decoded = JWT.decode(token, settings.jwt_secret[:secret], true, { algorithm: 'HS256' })
      user_id = decoded[0]['id']

      user = User.find(id: user_id)
      current_user = {
        :username => user[:username],
        :id => user[:id],
        :full_name => user[:full_name]
      }
      return current_user
    rescue JWT::DecodeError, JWT::ExpiredSignature, Sequel::NoMatchingRow => e
      puts "rescue current user: #{e.message}"
    end
  end

  def authenticate request = nil
    token = request.cookies['jwt']
    begin
      decoded_token = JWT.decode token, settings.jwt_secret[:secret], true, { algorithm: 'HS256' }
      @user = User[decoded_token.first['user_id']]
      true
    rescue JWT::ExpiredSignature, JWT::VerificationError, JWT::DecodeError => e
      redirect '/login'
    end
  end
end
