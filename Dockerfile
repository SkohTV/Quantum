FROM python:3.12-slim

WORKDIR /app

COPY requirements.txt .
COPY main.py .
COPY .env .
COPY src .

RUN pip install -r requirements.txt


CMD ["python", "main.py"]
