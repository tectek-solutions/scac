# Define build arguments
ARG ALPINE_VERSION=3.20
ARG FLUTTER_VERSION=2.24.5
ARG NGINX_VERSION=1.27.3

# Environment to install Flutter and build web
FROM alpine:${ALPINE_VERSION} AS builder

# Install dependencies
RUN apk add --no-cache curl openssl-dev git unzip bash coreutils libc6-compat

RUN addgroup -S flutter
RUN adduser -S flutter -G flutter
USER flutter

# Define variables
ARG FLUTTER_SDK=/home/flutter/flutter
ARG APP=/home/flutter/scac

# Clone the Flutter SDK
RUN mkdir -p $FLUTTER_SDK
RUN git clone https://github.com/flutter/flutter.git $FLUTTER_SDK
WORKDIR $FLUTTER_SDK
RUN git fetch
RUN git checkout $FLUTTER_VERSION

# Set up Flutter environment variables
ENV PATH="$FLUTTER_SDK/bin:$FLUTTER_SDK/bin/cache/dart-sdk/bin:${PATH}"

# Verify Flutter installation
RUN flutter doctor -v

# Copy the web application source code
RUN mkdir -p $APP
COPY --chown=flutter:flutter ./client $APP/client
WORKDIR $APP

# Build the web application
RUN cd client && flutter clean
RUN cd client && flutter pub get
RUN cd client && flutter build web

# Deploy using NGINX
FROM nginx:${NGINX_VERSION}-alpine${ALPINE_VERSION}

ARG APP=/home/flutter/scac

# Copy built web application to NGINX HTML directory
COPY --from=builder --chown=nginx:nginx $APP/client/build/web /var/www/client
COPY --from=builder --chown=root:root $APP/client/nginx.conf /etc/nginx/nginx.conf

# Expose and run NGINX
EXPOSE 8000
CMD ["nginx", "-g", "daemon off;"]
