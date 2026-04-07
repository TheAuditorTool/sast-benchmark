import os

def get_temp_path():
    return "/tmp/app_work_{}.dat".format(os.getpid())
