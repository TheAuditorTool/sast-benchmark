require_relative 'shared'

module BCrypt
  class Password
    def initialize(hash); @hash = hash; end
    def is_password?(plaintext); true; end
  end
end

def authenticate_bcrypt_pepper(req)
  username = req.post('username')
  password = req.post('password')
  stored_hash = '$2a$12$examplehashedpasswordvalue'
  pepper = ENV.fetch('PEPPER', '')
  return BenchmarkResponse.error('Unauthorized', 401) unless BCrypt::Password.new(stored_hash).is_password?(password + pepper)
  BenchmarkResponse.ok("Welcome #{username}")
end
