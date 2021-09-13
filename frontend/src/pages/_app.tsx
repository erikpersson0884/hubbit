import '../global-styles/styles.scss';
import '../global-styles/tables.scss';
import '../global-styles/groups.scss';

import React from 'react';

import { createClient, defaultExchanges, errorExchange, Provider } from 'urql';

import Footer from '../components/footer/Footer';
import Header from '../components/header/Header';

function HubbitApp({ Component, pageProps }) {
  const error = errorExchange({
    onError: error => {
      for (const e of error.graphQLErrors) {
        if (e.extensions['code'] === 'NOT_LOGGED_IN') {
          window.location.href = `${window.location.origin}/api/auth/gamma/login?from=${window.location.href}`;
        }
      }
    },
  });

  const client = createClient({
    // TODO(vidarm): Set this as an environment variable
    url: typeof window === 'undefined' ? 'http://hubbit-backend:8080' : '/api/graphql',
    exchanges: [error, ...defaultExchanges],
  });

  return (
    <Provider value={client}>
      <div className={'pageWrapper'}>
        <Header />
        <div className={'componentWrapper'}>
          <Component {...pageProps} />
        </div>
        <Footer />
      </div>
    </Provider>
  );
}

export default HubbitApp;
