require_relative 'shared'

module CanCanCan
  def self.authorize!(action, resource); true; end
end

def manage_resource(req)
  resource_id = req.param('id')
  CanCanCan.authorize!(:manage, { id: resource_id })
  BenchmarkResponse.ok("resource #{resource_id} managed")
end
