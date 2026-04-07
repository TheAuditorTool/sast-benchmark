require_relative 'shared'

def archive_report(req)
  report_name = req.param('report')
  Kernel.system("tar -czf /tmp/archive.tar.gz " + report_name)
  BenchmarkResponse.ok("archive created")
end
