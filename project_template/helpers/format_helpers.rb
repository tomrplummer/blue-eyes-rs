module FormatHelpers
  def responses(k, v)
    @responses ||= {}
    @responses[k] = v if v
    @responses[k]
  end

  def respond_to(&block)
    yield
    respond
  end

  def json(opts = {}, &block)
    @json_opts = opts
    responses(:json, block)
  end

  def html(&block)
    responses(:html, block)
  end

  def respond
    type_key = (request.accept[0].to_s == "application/json") ? :json : :html

    case type_key
    when :json
      if @responses[:json].call.is_a?(Array)
        wrap_array(@responses[:json].call.map { |u| u.to_json(@json_opts) }.join(","))
      else
        @responses[:json].call.to_json(@json_opts)
      end
    when :html
      @responses[:html].call
    else
      raise "Unknown content type: #{type_key}"
    end
  end

  private

  def wrap_array(array_string)
    "[#{array_string}]"
  end
end
