require_relative 'shared'

# vuln-code-snippet start ruby_sqli_pluck_safe
def list_active_subscriber_contacts(req)
  page = req.param('page', default: '1').to_i
  per_page = 100

  relation = FakeActiveRecord::Base.where(active: true)
  names_and_emails = relation.pluck(:name, :email)  # vuln-code-snippet safe-line ruby_sqli_pluck_safe

  offset = (page - 1) * per_page
  paged = Array(names_and_emails).slice(offset, per_page) || []
  contacts = paged.map { |row| { name: row[0], email: row[1] } }

  BenchmarkResponse.json({ contacts: contacts, page: page, per_page: per_page })
end
# vuln-code-snippet end ruby_sqli_pluck_safe
