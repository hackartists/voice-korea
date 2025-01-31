FROM ubuntu

WORKDIR /app

COPY . /app/dist

RUN apt-get update && apt-get install -y apt-utils ca-certificates

CMD ["dist/bootstrap", "--addr", "0.0.0.0", "--port", "8080"]

EXPOSE 8080
