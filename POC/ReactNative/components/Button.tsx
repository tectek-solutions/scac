import React from 'react';
import { StyleSheet, Pressable } from 'react-native';
import { ThemedText } from './ThemedText';

export default function Button({ title, onPress }) {
  return (
    <Pressable style={styles.button} onPress={onPress}>
      <ThemedText style={styles.text}>{title}</ThemedText>
    </Pressable>
  );
}

const styles = StyleSheet.create({
  button: {
    alignItems: 'center',
    justifyContent: 'center',
    paddingVertical: 12,
    paddingHorizontal: 32,
    borderRadius: 4,
    elevation: 3,
    backgroundColor: 'black',
  },
  text: {
    fontSize: 16,
    lineHeight: 21,
    fontWeight: 'bold',
    letterSpacing: 0.25,
    color: 'white',
  },
});
