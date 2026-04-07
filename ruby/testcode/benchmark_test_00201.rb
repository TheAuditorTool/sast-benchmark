require_relative 'shared'

FIXED_DELEGATES = %w[ping version status].freeze

class SafeProxy
  def ping    = 'pong'
  def version = '1.0'
  def status  = 'ok'

  def method_missing(name, *_args)
    return super unless FIXED_DELEGATES.include?(name.to_s)
    send(name)
  end

  def respond_to_missing?(name, include_private = false)
    FIXED_DELEGATES.include?(name.to_s) || super
  end
end

def handler(req)
  proxy = SafeProxy.new
  action = req.param('action')
  return BenchmarkResponse.bad_request('unknown') unless FIXED_DELEGATES.include?(action)
  result = proxy.send(action)
  BenchmarkResponse.ok(result)
end
