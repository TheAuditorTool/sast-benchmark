require_relative 'shared'

class AdminPanel
  def self.purge_inactive_users
    ['user_8', 'user_14', 'user_22']
  end
end

def purge_users(req)
  current_user = req.cookie('user_id')
  return BenchmarkResponse.error('unauthenticated', 401) unless current_user.present?
  purged = AdminPanel.purge_inactive_users
  BenchmarkResponse.json({ purged: purged })
end

class String
  def present?
    !empty?
  end
end
