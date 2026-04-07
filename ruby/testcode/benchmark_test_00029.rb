require_relative 'shared'

class CurrentUser
  attr_accessor :id, :role, :name

  def initialize(id, role)
    @id = id
    @role = role
    @name = "user_#{id}"
  end

  def update(attrs)
    attrs.each { |k, v| instance_variable_set(:"@#{k}", v) }
    self
  end
end

def update_profile(req)
  current_user = CurrentUser.new(req.cookie('user_id'), 'member')
  current_user.update(role: req.param('role'))
  BenchmarkResponse.ok('profile updated')
end
