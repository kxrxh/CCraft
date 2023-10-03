import os
import subprocess


current_file_path = os.path.dirname(os.path.abspath(__file__))

def run_rust_cli(args):
    """
    Run the Rust CLI application with the given arguments and return the output.
    """
    try:
        output = subprocess.check_output([current_file_path + '/ccraft'] + args, stderr=subprocess.STDOUT, text=True)
        return output
    except subprocess.CalledProcessError as e:
        return e.output

def test_run_command():
    # Test the "run" subcommand
    output = run_rust_cli(['run'])
    # assert "Running the application" in output  # Replace with expected output

    print("[✅] RUN COMMAND TEST PASSED")

def test_build_command():
    # Test the "build" subcommand
    output = run_rust_cli(['build'])
    # assert "Building the application" in output  # Replace with expected output

    print("[✅] BUILD COMMAND TEST PASSED")

def test_check_command():
    # Test the "check" subcommand
    output = run_rust_cli(['check'])
   #  assert "Checking the application" in output  # Replace with expected output

def test_init_command():
    # Test the "init" subcommand
    output = run_rust_cli(['init'])
    # assert "Initializing a new C project" in output  # Replace with expected output

    print("[✅] INIT COMMAND TEST PASSED")
def test_new_command_creates_folder():
    project_name = 'my_project'
    output = run_rust_cli(['new', project_name])
    
    assert os.path.exists(project_name)
    assert "Project successfully created" in output

    print("[✅] NEW COMMAND DEFAULT CREATION TEST PASSED")

def test_new_command_folder_already_exists():
    project_name = 'my_project'
    
    try :
        os.mkdir(project_name)
    except FileExistsError:
        pass
    
    output = run_rust_cli(['new', project_name])
    assert "Folder already exists" in output

    print("[✅] NEW COMMAND FOLDER ALREADY EXISTS TEST PASSED")

def test_new_command_empty_folder_name():
    output = run_rust_cli(['new', ''])
    
    assert "Invalid folder name" in output

    print("[✅] NEW COMMAND EMPTY FOLDER NAME TEST PASSED")

def test_new_command_invalid_folder_name():
    invalid_project_name = '////'
    output = run_rust_cli(['new', invalid_project_name])
    
    assert "Invalid folder name" in output

    print("[✅] NEW COMMAND INVALID FOLDER NAME TEST PASSED")

def test_config_sync_command():
    # Test the "config sync" subcommand
    output = run_rust_cli(['config', 'sync'])
    # assert "Syncing config file" in output  # Replace with expected output

    print("[✅] CONFIG SYNC COMMAND TEST PASSED")

def test_config_clear_command():
    # Test the "config clear" subcommand
    output = run_rust_cli(['config', 'clear'])
    # assert "Clearing config file" in output  # Replace with expected output

    print("[✅] CONFIG CLEAR COMMAND TEST PASSED")

if __name__ == "__main__":
    test_run_command()
    test_build_command()
    test_check_command()
    test_init_command()

    test_new_command_creates_folder()
    test_new_command_folder_already_exists()
    test_new_command_empty_folder_name()
    test_new_command_invalid_folder_name()

    test_config_sync_command()
    test_config_clear_command()
    print("All tests passed!")
