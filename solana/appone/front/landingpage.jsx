import React from 'react';

class LandingPage extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      name: '',
      image: null,
    };
  }

  handleNameChange = (event) => {
    this.setState({ name: event.target.value });
  };

  handleImageChange = (event) => {
    this.setState({ image: event.target.files[0] });
  };

  render() {
    return (
      <div>
        <form>
          <label>
            Name:
            <input type="text" value={this.state.name} onChange={this.handleNameChange} />
          </label>
          <br />
          <label>
            Image:
            <input type="file" onChange={this.handleImageChange} />
          </label>
        </form>
      </div>
    );
  }
}

export default LandingPage;
