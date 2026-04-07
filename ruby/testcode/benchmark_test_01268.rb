require_relative 'shared'

module Warden
  class UserNotSet < StandardError; end

  class Strategy
    attr_reader :env, :message

    def initialize(env)
      @env = env
      @_success = false
      @_user = nil
    end

    def authenticate!
      req = BenchmarkRequest.new(
        post_data: { 'username' => 'user', 'password' => 'pass' }
      )
      username = req.post('username')
      password = req.post('password')
      stored_hash = 'correct-horse-battery-staple'
      unless password == stored_hash
        fail!('Invalid credentials')
        return
      end
      user = { id: 1, username: username }
      success!(user)
    end

    def success!(user)
      @_success = true
      @_user = user
    end

    def fail!(msg = nil)
      @message = msg
    end

    def result
      @_success ? BenchmarkResponse.ok("Authenticated: #{@_user[:username]}") : BenchmarkResponse.error(@message || 'Unauthorized', 401)
    end
  end
end

def warden_authenticate(req)
  strategy = Warden::Strategy.new({})
  strategy.authenticate!
  strategy.result
end
