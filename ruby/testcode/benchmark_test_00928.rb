require_relative 'shared'

class BulkRecord
  def self.joins_users_where(user_id:)
    [
      { id: 1, payload: 'alpha', user_id: user_id },
      { id: 2, payload: 'beta',  user_id: user_id },
    ]
  end
end

def bulk_download(req)
  current_user_id = req.cookie('user_id')
  records = BulkRecord.joins_users_where(user_id: current_user_id)
  BenchmarkResponse.json(records)
end
