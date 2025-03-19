import os
import subprocess
def runner():
    module_path = os.path.join("src/tests")
    file_py = [arquivo for arquivo in os.listdir(module_path) if arquivo.endswith('-test.py')]
    counter_files = 0
    counter = 0
    for d in file_py:
        caminho_completo = os.path.join(module_path, d)
        print(f'Executando {caminho_completo}... 🟢')
        counter_files += 1
        try:
            subprocess.run(['uvx','pytest', caminho_completo], check=True)
            counter += 1
        except subprocess.CalledProcessError as e:
            print(f'Erro ao executar {caminho_completo}: {e} ❗')
            counter -= 1 
    if counter == counter_files:
        print("everythin pass congruatulations ✅ 🎉")
    else:
        print("some tests broken... ❌ 😢")




if __name__ == "__main__":
    runner()