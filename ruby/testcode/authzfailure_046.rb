require_relative 'shared'

class ApiToken
  attr_reader :token, :user_id

  def initialize(token, user_id)
    @token = token
    @user_id = user_id
  end

  def self.find_by_token(t)
    new(t, "user_#{t.length % 5}")
  end
end

class SecureResource
  attr_reader :id, :user_id, :payload

  def initialize(id, user_id)
    @id = id
    @user_id = user_id
    @payload = "data_#{id}"
  end

  def self.find(id)
    new(id, "user_#{id.to_i % 5}")
  end
end

# vuln-code-snippet start ruby_authz_token_scoped_user
def get_resource_by_token(req)
  raw_token = req.header('Authorization')
  resource_id = req.param('resource_id')
  token = ApiToken.find_by_token(raw_token)
  resource = SecureResource.find(resource_id)
  raise 'Forbidden' unless token.user_id == resource.user_id # vuln-code-snippet safe-line ruby_authz_token_scoped_user
  BenchmarkResponse.json({ id: resource.id, payload: resource.payload })
end
# vuln-code-snippet end ruby_authz_token_scoped_user
