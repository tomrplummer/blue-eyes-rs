class UsersService
  def initialize(id: nil, params: {}, current_user: nil)
    @id = id
    @password = params[:password]
    @username = params[:username]
    @params = params
    @current_user = current_user
  end

  def validate_new_user
    username_validation_error = validate_username
    return username_validation_error unless username_validation_error.nil?

    password_validation_error = validate_password
    return password_validation_error unless password_validation_error.nil?
  end

  def create_user
    user_validation_error = validate_new_user
    return user_validation_error unless user_validation_error.nil?

    password_hash = hash_password
    begin
      user = User.create({
        username: @username,
        password_hash: password_hash
      })
      {success: true, user: user}
    rescue
      {
        error: Err.server_error,
        message: "Unable able to create user."
      }
    end
  end

  def show_user
    access_error = check_access
    return access_error if access_error

    begin
      user = User.find(id: @id)
      puts "@USER: #{user}"
      if user.nil?
        user = User.new
        return {
          error: Err.not_found,
          message: "User not found",
          user: user
        }
      end

      return {success: true, user: user}
    rescue => e
      puts e
      {error: Err.server_error, message: e}
    end
  end

  def update_user
    access_error = check_access
    return access_error if access_error

    begin
      user = User.find(id: @id)
      return {error: Err.not_found} if user.nil?

      new_params = {full_name: @params[:full_name]}

      if user.username != @username
        username_validation_error = validate_username
        return username_validation_error unless username_validation_error.nil?

        new_params[:username] = @username
      end

      if @password && @password != ""
        password_validation_error = validate_password
        return password_validation_error unless password_validation_error.nil?

        new_params[:password_hash] = hash_password
      end

      result = user.update User.permitted(new_params)
      {success: true}
    rescue => e
      {error: Err.server_error, message: e}
    end
  end

  private

  def hash_password
    BCrypt::Password.create(@password)
  end

  def password_requirements
    "Password must be at least 8 characters and have the 3 out of 4: Upper case, lower case, number, symbol."
  end

  def validate_username
    return {error: Err.unproccessable_entity, message: "Username cannot be blank."} if @username.nil? || @username == ""
    user = User.find(username: @username)
    return {error: Err.unproccessable_entity, message: "Username in use."} unless user.nil?
  end

  def validate_password
    if @password.length < 8
      return {
        error: Err.unproccessable_entity,
        message: password_requirements
      }
    end

    complexity_score = 0
    complexity_score += 1 if /[A-Z]/.match?(@password)
    complexity_score += 1 if /[a-z]/.match?(@password)
    complexity_score += 1 if /[0-9]/.match?(@password)
    complexity_score += 1 if /[^A-Za-z0-9]/.match?(@password)

    if complexity_score < 3
      return {
        error: Err.unproccessable_entity,
        message: password_requirements
      }
    end
  end

  def check_access
    return {error: Err.unauthorized, message: "Not logged in"} unless @current_user
    return {error: Err.access_denied, message: "You do not have access to the page"} unless @id && @current_user[:id] == @id.to_i
  end
end
