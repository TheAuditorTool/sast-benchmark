import tempfile

def get_temp_path():
    fd, path = tempfile.mkstemp(prefix="app_work_", suffix=".dat")
    return fd, path
