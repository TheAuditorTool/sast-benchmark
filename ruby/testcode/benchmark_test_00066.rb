require_relative 'shared'

def list_accounts_by_status(req)
  status = req.param('status')
  page = req.param('page', default: '1').to_i
  per_page = 25
  results = FakeActiveRecord::Base.where(status: status)
  data = results.to_a
  offset = (page - 1) * per_page
  BenchmarkResponse.json({ accounts: data.slice(offset, per_page) || [], page: page })
end
