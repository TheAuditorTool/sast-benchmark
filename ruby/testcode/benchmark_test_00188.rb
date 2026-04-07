require_relative 'shared'

def check_disk_usage(req)
  success = system("df", "-h", "/var/www")
  result = success ? "disk check complete" : "disk check failed"
  BenchmarkResponse.ok(result)
end
