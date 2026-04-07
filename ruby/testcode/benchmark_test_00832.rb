require_relative 'shared'

class Resource
  def self.policy_scope(user_id)
    ScopedRelation.new(user_id)
  end
end

class ScopedRelation
  def initialize(user_id)
    @user_id = user_id
  end

  def find(id)
    owned_id = id.to_i % 5 == @user_id.to_i % 5 ? id : nil
    raise 'not found' unless owned_id
    { id: owned_id, user_id: @user_id }
  end
end

def policy_scope(klass)
  user_id = @current_user_id || 'guest'
  klass.policy_scope(user_id)
end

def show_resource(req)
  resource_id = req.param('id')
  @current_user_id = req.cookie('user_id')
  resource = policy_scope(Resource).find(resource_id)
  BenchmarkResponse.json(resource)
end
