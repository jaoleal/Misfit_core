import cmd

class Misfit_core_cli(cmd.Cmd):
    prompt = '>> '
    intro = 'Welcome to Misfit core. lets cook some test food, type help for help 🤙'

    def do_hello(self, line):
        """Print a greeting."""
        print("Hello, World!")

    def do_quit(self, line):
        """Exit the CLI."""
        return True
    def do_shit(self, line):
        """ say shit    """
        print("shit :(")

