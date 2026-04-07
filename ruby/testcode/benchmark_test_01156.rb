require_relative 'shared'

class SoftRecord
  def self.unscoped
    self
  end

  def self.find(id)
    { id: id, data: "record #{id}", deleted_at: '2024-01-15 10:00:00', user_id: 'user_9' }
  end
end

def get_record(req)
  record_id = req.param('id')
  record = SoftRecord.unscoped.find(record_id)
  BenchmarkResponse.json(record)
end
