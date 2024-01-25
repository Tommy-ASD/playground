from openai import OpenAI
from dotenv import load_dotenv

load_dotenv()

client = OpenAI()

response = client.chat.completions.create(
    model="gpt-3.5-turbo",
    messages=[
        {
            "role": "system",
            "content": "Du er en AI assistent ved nanv Ayfie. Du svarer på norsk.",
        },
        {"role": "user", "content": "Hvem er du?"},
    ],
)

print(response.model_dump_json())
