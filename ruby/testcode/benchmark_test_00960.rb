require_relative 'shared'

class User
  attr_accessor :id, :name, :email

  def initialize(id, name, email)
    @id = id
    @name = name
    @email = email
  end

  def self.find(id)
    new(id, "user_#{id}", "user#{id}@example.com")
  end

  def update(attrs)
    attrs.each { |k, v| send(:"#{k}=", v) if respond_to?(:"#{k}=") }
    self
  end
end

def update_user(req)
  target_id = req.param('id')
  attrs = { name: req.param('name'), email: req.param('email') }
  User.find(target_id).update(attrs)
  BenchmarkResponse.ok('updated')
end
