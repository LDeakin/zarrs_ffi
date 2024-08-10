```bash
mkdir build && cd build
cmake -DCMAKE_BUILD_TYPE=Release .. && make
./main
```

```bash
(cd .. && docker run --rm -it $(docker build -q -f cmake_project/Dockerfile .))
```
