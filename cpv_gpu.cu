#include <stdio.h>
#include <cuda_runtime.h>


__global__ void cpv__(const float *vec, float *res, int N) {
    int tid = threadIdx.x + blockIdx.x * blockDim.x;
    res[i] = int(vec[i] > 0);
}

extern "C" {
    float *gpu_cpv(float *vec, size_t N) {
        cudaError_t err = cudaSuccess;
        float *dev_vec, *dev_res, *res;
        int threadsPerBlock = 256;
        int blocksPerGrid = (N + threadsPerBlock - 1) / threadsPerBlock;
        res = new float[blocksPerGrid];

        err = cudaMalloc((void **) &dev_vec, N * sizeof(float));
        if (err != cudaSuccess)
        {
            fprintf(stderr, "Failed to allocate device vector (error code %s)!\n", cudaGetErrorString(err));
            exit(EXIT_FAILURE);
        }

        err = cudaMalloc((void **) &dev_res, blocksPerGrid * sizeof(float));
        if (err != cudaSuccess)
        {
            fprintf(stderr, "Failed to allocate device vector (error code %s)!\n", cudaGetErrorString(err));
            exit(EXIT_FAILURE);
        }

        err = cudaMemcpy(dev_vec, vec, N * sizeof(float), cudaMemcpyHostToDevice);
        if (err != cudaSuccess)
        {
            fprintf(stderr, "Failed to copy vector from host to device (error code %s)!\n", cudaGetErrorString(err));
            exit(EXIT_FAILURE);
        }

        cpv__<<<blocksPerGrid, threadsPerBlock>>>(dev_vec, dev_res, N);
        err = cudaGetLastError();
        if (err != cudaSuccess)
        {
            fprintf(stderr, "Failed to launch cpv kernel (error code %s)!\n", cudaGetErrorString(err));
            exit(EXIT_FAILURE);
        }

        err = cudaMemcpy(res, dev_res, blocksPerGrid * sizeof(float), cudaMemcpyDeviceToHost);
        if (err != cudaSuccess)
        {
            fprintf(stderr, "Failed to copy vector from device to host (error code %s)!\n", cudaGetErrorString(err));
            exit(EXIT_FAILURE);
        }

        err = cudaFree(dev_vec);
        if (err != cudaSuccess)
        {
            fprintf(stderr, "Failed to free device vector (error code %s)!\n", cudaGetErrorString(err));
            exit(EXIT_FAILURE);
        }
//        cudaFree(dev_res);

        err = cudaDeviceReset();
        if (err != cudaSuccess)
        {
            fprintf(stderr, "Failed to deinitialize the device! error=%s\n", cudaGetErrorString(err));
            exit(EXIT_FAILURE);
        }

        return res;
    }
}