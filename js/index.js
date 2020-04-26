const rust = import('../pkg');

rust
  .then(m => {
    console.log('Module boot success');
    m.start();
  })
  .catch(console.error);
