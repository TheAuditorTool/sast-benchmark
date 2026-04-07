require_relative 'shared'

class ApiKey
  def self.find_by(conditions)
    key = conditions[:token]
    key ? { token: key, user_id: 'user_3', valid: true } : nil
  end
end

class User
  def self.find(id)
    { id: id, email: "#{id}@example.com", balance: 5_000 }
  end
end

def get_user_by_api_key(req)
  api_key = req.header('X-Api-Key')
  token_record = ApiKey.find_by(token: api_key)
  return BenchmarkResponse.error('invalid key', 401) unless token_record && token_record[:valid]
  target_user_id = req.param('user_id')
  user = User.find(target_user_id)
  BenchmarkResponse.json(user)
end
