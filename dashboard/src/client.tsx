import ApolloClient from 'apollo-boost';

const client = new ApolloClient({
  uri: 'http://localhost:5600/admin/graphql',
});

export default client;
