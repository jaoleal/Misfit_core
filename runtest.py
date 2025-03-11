import os
import subprocess


directory = './src/tests/'

file_py = [arquivo for arquivo in os.listdir(diretorio) if arquivo.endswith('-test.py')]


for arquivo in arquivos_py:
    caminho_completo = os.path.join(diretorio, arquivo)
    print(f'Executando {caminho_completo}...')
    try:
        subprocess.run(['uvx','pytest', caminho_completo], check=True)
    except subprocess.CalledProcessError as e:
        print(f'Erro ao executar {caminho_completo}: {e}')