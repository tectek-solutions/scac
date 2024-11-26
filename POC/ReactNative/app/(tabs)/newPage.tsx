import React from 'react';
import { View, Text, StyleSheet } from 'react-native';

export default function NewPage() {
  return (
    <View style={styles.container}>
      <Text style={styles.text}>This is the new page</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  text: {
    fontSize: 20,
    color: 'white'
  },
});