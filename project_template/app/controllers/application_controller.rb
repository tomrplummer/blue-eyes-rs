require "sinatra/base"
require "sinatra/reloader" if development?
require "sinatra/flash"
require "logger"
require "active_support"
require_relative "../../helpers/error"

class ApplicationController < Sinatra::Base
  use Rack::MethodOverride
  extend ActiveSupport::Inflector
  register Sinatra::Flash
  include Err
  enable :sessions

  @logger = Logger.new $stdout

  configure do
    set :jwt_secret, secret: ENV["JWT_SECRET"]
  end

  configure :development do
    register Sinatra::Reloader
  end

  set :public_folder, File.join(root, "..", "public")

  set :views, -> {
    File.expand_path(
      "../../app/views/",
      File.dirname(__FILE__)
    )
  }

  post "/unauthenticated" do
    redirect "/login"
  end
end
