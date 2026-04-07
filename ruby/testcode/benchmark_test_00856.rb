require_relative 'shared'

class UserService
  def activate(id)    = "activated:#{id}"
  def deactivate(id)  = "deactivated:#{id}"
end

def reflect_public_send(req)
  action = req.param('action')
  id = req.param('id')
  service = UserService.new
  result = service.public_send(action, id)
  BenchmarkResponse.ok(result.to_s)
end
