if [ ! -f tokenizer.json ]; then
    wget https://huggingface.co/Gadersd/whisper-burn/resolve/main/tiny_en/tokenizer.json
fi

cd python

if [ ! -f tiny.en.pt ]; then
    wget https://openaipublic.azureedge.net/main/whisper/models/d3dd57d32accea0b295c96e26691aa14d8822fac7d9d27d5dc00b4ca2826dd03/tiny.en.pt
fi

pip install -r requirements.txt
python3 dump.py tiny.en.pt tiny_en
mv tiny_en ../

cd ../

cargo run --release --bin convert tiny_en

