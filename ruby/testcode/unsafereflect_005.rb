require_relative 'shared'

class UserService
  def activate(id)    = "activated:#{id}"
  def deactivate(id)  = "deactivated:#{id}"
end

# vuln-code-snippet start ruby_reflect_public_send
def reflect_public_send(req)
  action = req.param('action')
  id = req.param('id')
  service = UserService.new
  result = service.public_send(action, id) # vuln-code-snippet vuln-line ruby_reflect_public_send
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_reflect_public_send
