from bark import SAMPLE_RATE, generate_audio, preload_models
from scipy.io.wavfile import write as write_wav
from IPython.display import Audio

# download and load all models
preload_models()

# generate audio from text
text_prompt = """
2. GPUOpen Libraries AMD provides GPUOpen libraries and tools that are designed to replace specific CUDA functionalities with ROCm-compatible equivalents. These libraries include things like MIOpen (analogous to cuDNN) for deep learning, rocBLAS (analogous to cuBLAS) for linear algebra, and rocFFT (analogous to cuFFT) for Fourier transforms. Developers can use these libraries to replace CUDA-specific calls in their code with ROCm-compatible versions.
"""
audio_array = generate_audio(text_prompt)

# save audio to disk
write_wav("bark_generation.wav", SAMPLE_RATE, audio_array)
  
# play text in notebook
Audio(audio_array, rate=SAMPLE_RATE)