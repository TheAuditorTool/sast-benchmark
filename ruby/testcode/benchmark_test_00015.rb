require_relative 'shared'

class Service
  def start; 'started'; end
  def stop; 'stopped'; end
  def restart; 'restarted'; end
end

def manage_service(req)
  action = req.param('action')
  svc = Service.new
  allowed = %w[start stop restart]
  if svc.respond_to?(action) && allowed.include?(action)
    BenchmarkResponse.ok(svc.public_send(action))
  else
    BenchmarkResponse.bad_request('invalid action')
  end
end
