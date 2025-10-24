// React state patterns
import React, { useState, useReducer, useEffect, useContext } from 'react';

function Component() {
    const [count, setCount] = useState(0);
    const [state, dispatch] = useReducer(reducer, initialState);
    const context = useContext(MyContext);
    
    useEffect(() => {
        // Side effect
    }, [count]);
    
    return <div>{count}</div>;
}

// Class component state
class ClassComponent extends React.Component {
    constructor(props) {
        super(props);
        this.state = { count: 0 };
    }
    
    handleClick = () => {
        this.setState({ count: this.state.count + 1 });
    }
}

// Redux patterns
import { createStore, combineReducers, applyMiddleware } from 'redux';

const store = createStore(rootReducer, applyMiddleware(thunk));
store.dispatch(action);
const state = store.getState();
store.subscribe(listener);

// Observable patterns
import { Observable, Subject, BehaviorSubject } from 'rxjs';

const subject = new Subject();
const observable = new Observable(subscriber => {
    subscriber.next(value);
    subscriber.complete();
});

observable.subscribe(value => console.log(value));
subject.next(data);

// Event emitter patterns
const EventEmitter = require('events');
const emitter = new EventEmitter();
emitter.on('event', handler);
emitter.emit('event', data);
emitter.off('event', handler);
