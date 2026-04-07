require_relative 'shared'

class Report
  def self.all
    [
      { id: 1, title: 'Q1 Revenue', user_id: 'user_1' },
      { id: 2, title: 'Salary Data', user_id: 'user_2' },
      { id: 3, title: 'HR Incidents', user_id: 'user_3' },
    ]
  end
end

# vuln-code-snippet start ruby_authz_unscoped_report
def list_reports(req)
  reports = Report.all # vuln-code-snippet vuln-line ruby_authz_unscoped_report
  BenchmarkResponse.json(reports)
end
# vuln-code-snippet end ruby_authz_unscoped_report
