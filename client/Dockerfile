ARG NGINX_VERSION=1.27.3

# Environment to install Flutter and build web
FROM debian:bookworm-slim AS builder

# Define build arguments
ARG FLUTTER_VERSION=3.27.2

ARG GRADLE_VERSION=6.3
ARG ANDROID_API_LEVEL=29
ARG ANDROID_BUILD_TOOLS_LEVEL=29.0.3
ARG ANDROID_NDK_VERSION=21.1.6352462

VOLUME [ "/home/flutter" ]

# Install dependencies
RUN apt-get update
RUN apt install -qq -y curl git unzip xz-utils zip libglu1-mesa default-jre default-jdk libglu1 libpulse-dev libasound2 libc6 libstdc++6 libx11-6 libx11-xcb1 libxcb1 libxcomposite1 libxcursor1 libxi6 libxtst6 libnss3 wget

# Set up Flutter environment variables
ENV PATH="$FLUTTER_SDK/bin:$FLUTTER_SDK/bin/cache/dart-sdk/bin:${PATH}"

# Download gradle, install gradle and gradlew
RUN wget -q https://services.gradle.org/distributions/gradle-${GRADLE_VERSION}-bin.zip -P /tmp
RUN unzip -q -d /opt/gradle /tmp/gradle-${GRADLE_VERSION}-bin.zip
RUN mkdir /opt/gradlew 
RUN /opt/gradle/gradle-${GRADLE_VERSION}/bin/gradle wrapper --gradle-version ${GRADLE_VERSION} --distribution-type all -p /opt/gradlew
RUN /opt/gradle/gradle-${GRADLE_VERSION}/bin/gradle wrapper -p /opt/gradlew

# Download commandlinetools, install packages and accept all licenses
RUN mkdir /opt/android
RUN mkdir /opt/android/cmdline-tools
RUN wget -q 'https://dl.google.com/android/repository/commandlinetools-linux-6200805_latest.zip' -P /tmp
RUN unzip -q -d /opt/android/cmdline-tools /tmp/commandlinetools-linux-6200805_latest.zip
RUN yes Y | /opt/android/cmdline-tools/tools/bin/sdkmanager --install "build-tools;${ANDROID_BUILD_TOOLS_LEVEL}" "platforms;android-${ANDROID_API_LEVEL}" "platform-tools" "ndk;${ANDROID_NDK_VERSION}"
RUN yes Y | /opt/android/cmdline-tools/tools/bin/sdkmanager --licenses

# Create a non-root user
RUN groupadd flutter
RUN useradd -m -g flutter flutter
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

# Environment variables to be used for build
ENV GRADLE_HOME=/opt/gradle/gradle-$GRADLE_VERSION
ENV ANDROID_HOME=/opt/android
ENV ANDROID_NDK_HOME=${ANDROID_HOME}/ndk/${ANDROID_NDK_VERSION}
ENV PATH "$PATH:$GRADLE_HOME/bin:/opt/gradlew:$ANDROID_HOME/emulator:$ANDROID_HOME/cmdline-tools/tools/bin:$ANDROID_HOME/platform-tools:${ANDROID_NDK_HOME}"
ENV LD_LIBRARY_PATH "$ANDROID_HOME/emulator/lib64:$ANDROID_HOME/emulator/lib64/qt/lib"

# Verify Flutter installation
RUN flutter doctor -v

# Copy the web application source code
RUN mkdir -p $APP
COPY --chown=flutter:flutter ./client $APP/client
WORKDIR $APP

# Build the web application
RUN cd client && flutter clean
RUN cd client && flutter pub get
RUN cd client && flutter build web --release --dart-define=API_URL=${API_URL}
RUN cd client && flutter build apk --dart-define=API_URL=${API_URL}

# Deploy using NGINX
FROM nginx:${NGINX_VERSION}-bookworm

ARG APP=/home/flutter/scac

# Copy built web application to NGINX HTML directory
COPY --from=builder --chown=nginx:nginx $APP/client/build/web /var/www/client
COPY --from=builder --chown=root:root $APP/client/nginx.conf /etc/nginx/nginx.conf

# Expose and run NGINX
EXPOSE 8000
CMD ["nginx", "-g", "daemon off;"]
