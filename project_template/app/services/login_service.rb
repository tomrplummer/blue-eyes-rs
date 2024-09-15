require "bcrypt"
require "jwt"

class LoginService
  def initialize(username, password, jwt_secret)
    @username = username
    @password = password
    @jwt_secret = jwt_secret
  end

  def call
    begin
      user = User.first(username: @username)
      if user && BCrypt::Password.new(user.password_hash) == @password
        token = generate_token(user)
        {success: true, token: token, user: user, status: 200}
      else
        {message: "Invalid username or password", error: Err.unproccessable_entity}
      end
    rescue => err
      {
        error: Err.server_error,
        message: err
      }
    end
  end

  private

  def generate_token(user)
    payload = {
      id: user[:id],
      username: user[:username],
      full_name: user[:full_name]
    }

    JWT.encode(payload, @jwt_secret, "HS256")
  end
end
