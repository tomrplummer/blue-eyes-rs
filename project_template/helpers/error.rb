require "haml"

module Err
  class Error
    attr_reader :status, :type
    def initialize(type, status)
      @type = type
      @status = status
    end

    def ==(other)
      @type == other
    end

    def eql?(other)
      self == other
    end

    def hash
      @type.hash
    end
  end

  def self.access_denied
    Error.new :access_denied, 403
  end

  def self.unauthorized
    Error.new :unauthorized, 401
  end

  def self.not_found
    Error.new :not_found, 404
  end

  def self.unproccessable_entity
    Error.new :unprocessable_entity, 422
  end

  def self.server_error
    Error.new :server_error, 500
  end

  def set_handler(k, v)
    @handlers ||= {}

    @handlers[k.is_a?(Error) ? k.type.to_sym : k.to_sym] = v unless k.nil? || v.nil?
  end

  def get_handler(k)
    @handlers ||= {}
    @handlers[:access_denied] = -> {
      status 401
      haml :access_denied
    } unless @handlers[:access_denied]
    @handlers[:server_error] = -> {
      status 500
      haml :error
    } unless @handlers[:server_error]

    @handlers[k.is_a?(Error) ? k.type : k] unless @handlers.nil? || k.nil?
  end

  def recover(type, &block)
    if type.is_a? Array
      type.each do |t|
        set_handler(t, block)
      end
    else
      set_handler(type, block)
    end
  end

  def error_response(error)
    return unless error.is_a?(Error) || error == :rest
    yield
    handle(error)
  end

  def handle(error)
    status (error.status) unless error.nil?
    error_handler = get_handler(error.is_a?(Error) ? error.type : error)
    error_handler ||= get_handler(:rest)
    return unless error_handler && error.is_a?(Error)

    response = error_handler.call
    throw :halt, response
  end
end
