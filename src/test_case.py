import os
import subprocess
from importlib import import_module

class Testrunner:
    def runner():
        module_path = import_module('./src/tests/')
        file_py = [arquivo for arquivo in os.listdir(module_path) if arquivo.endswith('-test.py')]


        for arquivo in file_py:
            caminho_completo = os.path.join(module_path, arquivo)
            print(f'Executando {caminho_completo}...')
            try:
                subprocess.run(['uvx','pytest', caminho_completo], check=True)
            except subprocess.CalledProcessError as e:
                print(f'Erro ao executar {caminho_completo}: {e}')

if __name__ == "__main__":
    Testrunner.runner()