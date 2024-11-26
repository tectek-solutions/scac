import { Image, StyleSheet, Button } from 'react-native';

import { GestureHandlerRootView } from 'react-native-gesture-handler';
import { HelloWave } from '@/components/HelloWave';
import ParallaxScrollView from '@/components/ParallaxScrollView';
import { ThemedText } from '@/components/ThemedText';
import { ThemedView } from '@/components/ThemedView';
import axios from 'axios';
import React, { useState, useEffect, useRef } from 'react';
import { ScrollView } from 'react-native-gesture-handler';
import Animated, {
  useSharedValue,
  useAnimatedStyle,
  withTiming,
  withRepeat,
  withSequence,
} from 'react-native-reanimated';

export default function HomeScreen() {
  const [data, setData] = useState([]);
  const [type, setType] = useState('');

  const spinValue = useSharedValue(0);

  spinValue.value = withRepeat(
    withSequence(withTiming(360, { duration: 10000 }), withTiming(0, { duration: 0 })),
    -1
  );

  const spin = useAnimatedStyle(() => ({
    transform: [{ rotate: `${spinValue.value}deg` }],
  }));


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

  return (
    <GestureHandlerRootView style={{ flex: 1 }}>
    <ParallaxScrollView
      headerBackgroundColor={{ light: '#A1CEDC', dark: '#1D3D47' }}
      headerImage={
        <Animated.View
          style={spin}
        >
          <Image
          source={require('@/assets/images/react-logox3.png')}
          style={styles.reactLogo}
          />
        </Animated.View>
      }>
      <ThemedView style={styles.titleContainer}>
        <ThemedText type="title">Welcome!</ThemedText>
        <HelloWave />
      </ThemedView>
      <ThemedText type="default">Use the buttons below to make API requests.</ThemedText>
      <ThemedView style={styles.stepContainer}>
        <ThemedText type="subtitle">GET Method:</ThemedText>
        <Button title="GET" onPress={getMethod} />
      </ThemedView>
      <ThemedView style={styles.stepContainer}>
        <ThemedText type="subtitle">POST Method:</ThemedText>
        <Button title="POST" onPress={postMethod} />
      </ThemedView>
      <ThemedView style={styles.stepContainer}>
        <ThemedText type="subtitle">PUT Method:</ThemedText>
        <Button title="PUT" onPress={putMethod} />
      </ThemedView>
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
    </ParallaxScrollView>
    </GestureHandlerRootView>
  );
}

const styles = StyleSheet.create({
  titleContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 8,
  },
  stepContainer: {
    gap: 8,
    marginBottom: 8,
  },
  reactLogo: {
    alignSelf: 'center',
    width: 200,
    height: 200
  },
  result: {
    maxHeight: 200,
    overflowY: 'scroll',
  },
});
