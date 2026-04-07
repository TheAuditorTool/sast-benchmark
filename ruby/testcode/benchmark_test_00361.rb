require_relative 'shared'

require 'fileutils'

def create_project_workspace(req)
  project_name = req.param('project_name')
  FileUtils.mkdir_p("/projects/#{project_name}/data")
  BenchmarkResponse.ok("Workspace created")
end
