import torch
from parler_tts import ParlerTTSForConditionalGeneration
from transformers import AutoTokenizer
import soundfile as sf

device = "cuda:0" if torch.cuda.is_available() else "cpu"

model = ParlerTTSForConditionalGeneration.from_pretrained("parler-tts/parler-tts-mini-v1").to(device)
tokenizer = AutoTokenizer.from_pretrained("parler-tts/parler-tts-mini-v1")

prompt = """You might think so, but ScreenX is designed in a way that you dont actually need to turn your head much. The main action and focus of the movie still take place on the front screen. The side projections are more about enhancing peripheral vision, creating a more immersive environment rather than requiring you to actively look to the sides.

The idea is for the side screens to complement the main visuals, drawing you deeper into the scene without distracting you from the primary action. Your brain naturally picks up on the extra visual information, which enhances the sense of immersion, but the content on the side walls is typically not crucial to the plot—it's more atmospheric, adding to the experience without requiring constant attention."""
description = "A female speaker delivers a slightly expressive and animated speech with a moderate speed and pitch. The recording is of very high quality, with the speaker's voice sounding clear and very close up."

input_ids = tokenizer(description, return_tensors="pt").input_ids.to(device)
prompt_input_ids = tokenizer(prompt, return_tensors="pt").input_ids.to(device)

generation = model.generate(input_ids=input_ids, prompt_input_ids=prompt_input_ids)
audio_arr = generation.cpu().numpy().squeeze()
sf.write("parler_tts_out_mini.wav", audio_arr, model.config.sampling_rate)
