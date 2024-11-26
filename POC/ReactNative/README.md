# Welcome to your Expo app 👋

This is an [Expo](https://expo.dev) project created with [`create-expo-app`](https://www.npmjs.com/package/create-expo-app).

## Get started

1. Install dependencies

   ```bash
   npm install
   ```

2. Start the app

   ```bash
    npx expo start
   ```

In the output, you'll find options to open the app in a

- [development build](https://docs.expo.dev/develop/development-builds/introduction/)
- [Android emulator](https://docs.expo.dev/workflow/android-studio-emulator/)
- [iOS simulator](https://docs.expo.dev/workflow/ios-simulator/)
- [Expo Go](https://expo.dev/go), a limited sandbox for trying out app development with Expo

You can start developing by editing the files inside the **app** directory. This project uses [file-based routing](https://docs.expo.dev/router/introduction).

## Get a fresh project

When you're ready, run:

```bash
npm run reset-project
```

This command will move the starter code to the **app-example** directory and create a blank **app** directory where you can start developing.

## Learn more

To learn more about developing your project with Expo, look at the following resources:

- [Expo documentation](https://docs.expo.dev/): Learn fundamentals, or go into advanced topics with our [guides](https://docs.expo.dev/guides).
- [Learn Expo tutorial](https://docs.expo.dev/tutorial/introduction/): Follow a step-by-step tutorial where you'll create a project that runs on Android, iOS, and the web.

## Join the community

Join our community of developers creating universal apps.

- [Expo on GitHub](https://github.com/expo/expo): View our open source platform and contribute.
- [Discord community](https://chat.expo.dev): Chat with Expo users and ask questions.


### Install and Configure EAS

With the use of eas: Expo Application Services (EAS) is a cloud build service used to build and publish React Native apps.

1. Install eas:
```
npm install -g eas-cli
```

2. Login with your account:
```
eas login
```

3. Init your project:
```
eas init
```

4. Start the configuration:
```
eas build:configure
```

5. In the eas.json, do this to have an .apk instead of .aab:
```
{
  "cli": {
    "version": ">= 13.3.0",
    "appVersionSource": "remote"
  },
  "build": {
    "development": {
      "developmentClient": true,
      "distribution": "internal"
    },
    "preview": {
      "distribution": "internal"
    },
    "production": {
      "autoIncrement": true,
      "android": {
        "buildType": "apk"
      }
    }
  },
  "submit": {
    "production": {}
  }
}
```

### Initialise folders for Android/IOS


1. Create folders for android and ios:
```
npx expo prebuild
```

2. Modify the android/gradle.properties:

```
# Default value: -Xmx512m -XX:MaxMetaspaceSize=256m
org.gradle.jvmargs=-Xmx2048m -XX:MaxMetaspaceSize=512m
```

### Build an apk locally

1. Download Android Studio

2. Go to SDK Manager > SDK Tools > Install NDK (Side by Side)

3. Exports env variable in your ~/.bashrc or ~/.zshrc or other:
```
   export ANDROID_HOME=path/to/your/android_studio/sdk_dir/
   export PATH=$ANDROID_HOME/tools:$ANDROID_HOME/tools/bin:$ANDROID_HOME/platform-tools:$PATH
   export ANDROID_NDK_HOME=path/to/your/android_studio/sdk/ndk_dir/
```

4. Source your .rc file:
```
source .bashrc
```

5. Finally, execute the command:
```
eas build --platform android --local
```

### Test the apk

1. In Android Studio, go to Virtual Device Manager

2. Select a phone

3. Start the emulator of the phone

4. Drag and drop the apk in the emulator

5. ENJOY !

### Developer documentation

For this project we have use the `axios` library for HTTP requests.
Axios supports all HTTP methods (GET, POST, PUT, DELETE, etc.), handles errors automatically, parses the response data in JSON.

### HTTP Requests

- GET Request: Fetches data from the API and sets it in the state.
const getMethod = () => {
      axios.get('https://jsonplaceholder.typicode.com/posts')
      .then((response) => {
          setData(response.data);
          setType('GET');
      })
      .catch((error) => {
          console.log(error);
      });
  }

- POST Request: Sends data to the API and updates the state with the response.
  const postMethod = () => {
      axios.post('https://jsonplaceholder.typicode.com/posts', {
          title: 'foo',
          body: 'bar',
          userId: 1
      })
      .then((response) => {
          setData([response.data]);
          setType('POST');
      })
      .catch((error) => {
          console.log(error);
      });
  }

- PUT Request: Updates data on the API and updates the state with the response.
  const putMethod = () => {
      axios.put('https://jsonplaceholder.typicode.com/posts/1', {
          id: 1,
          title: 'Test put',
          body: 'lorem ipsum',
          userId: 1
      })
      .then((response) => {
          setData([response.data]);
          setType('PUT');
      })
      .catch((error) => {
          console.log(error);
      });
  }

### State Management
State Variables:
    data: Stores the response data from the API.
    type: Stores the type of the last HTTP request made.

### Example Usage

In the Home component, buttons are provided to trigger each type of HTTP request, and the results are displayed based on the request type.
<ThemedView style={styles.stepContainer}>
        <ScrollView style={styles.result}>
          <ThemedText type="subtitle"> Result:</ThemedText>
            {type === 'GET' && (
              <ThemedView>
                  <ThemedText type="title">GET Results:</ThemedText>
                  {data.map((item, index) => (
                    <ThemedView key={index} style={styles.stepContainer}>
                      <ThemedText type="subtitle">Title: {item.title}</ThemedText>
                      <ThemedText type="default">Description: {item.body}</ThemedText>
                    </ThemedView>
                  ))}
              </ThemedView>
            )}
            {type === 'POST' && (
              <ThemedView>
                  <ThemedText type="title">POST Result:</ThemedText>
                  {data.map((item, index) => (
                    <ThemedView key={index} style={styles.stepContainer}>
                      <ThemedText type="subtitle">Title: {item.title}</ThemedText>
                      <ThemedText type="default">Description: {item.body}</ThemedText>
                    </ThemedView>
                  ))}
              </ThemedView>
            )}
            {type === 'PUT' && (
              <ThemedView>
                  <ThemedText type="title">PUT Result:</ThemedText>
                  {data.map((item, index) => (
                    <ThemedView key={index} style={styles.stepContainer}>
                        <ThemedText type="subtitle">Title: {item.title}</ThemedText>
                        <ThemedText type="default">Description: {item.body}</ThemedText>
                    </ThemedView>
                  ))}
              </ThemedView>
            )}
        </ScrollView>
      </ThemedView>