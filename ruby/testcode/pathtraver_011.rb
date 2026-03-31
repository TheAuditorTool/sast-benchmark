require_relative 'shared'

require 'fileutils'

# vuln-code-snippet start ruby_pt_mkdir_user
def create_project_workspace(req)
  project_name = req.param('project_name')
  FileUtils.mkdir_p("/projects/#{project_name}/data") # vuln-code-snippet vuln-line ruby_pt_mkdir_user
  BenchmarkResponse.ok("Workspace created")
end
# vuln-code-snippet end ruby_pt_mkdir_user
