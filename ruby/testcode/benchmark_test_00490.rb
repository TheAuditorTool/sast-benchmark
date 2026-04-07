require_relative 'shared'

class Record
  def self.all
    [
      { id: 1, data: 'record_alpha', user_id: 'user_1' },
      { id: 2, data: 'record_beta',  user_id: 'user_2' },
      { id: 3, data: 'record_gamma', user_id: 'user_3' },
    ]
  end
end

def bulk_export(req)
  current_user = req.cookie('user_id')
  return BenchmarkResponse.error('unauthenticated', 401) unless current_user.present?
  records = Record.all
  BenchmarkResponse.json(records)
end

class String
  def present?
    !empty?
  end
end
