// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

/**
 * @title EventDistributor
 * @dev Smart contract for managing validated participants and distributing funds
 * @notice Equivalent to Stellar Soroban EventDistributor but for Celo/EVM
 */
contract EventDistributor is Ownable, ReentrancyGuard {
    
    // ========== STRUCTS ==========
    
    struct Human {
        address walletAddress;
        string ipfsHash;        // IPFS hash of the user's image
        bool validated;
        bool exists;
    }
    
    struct Event {
        uint256 id;
        string name;
        address[] participants;
        bool exists;
    }
    
    // ========== STATE VARIABLES ==========
    
    mapping(address => Human) public humans;
    address[] public humanAddresses;
    
    mapping(uint256 => Event) public events;
    uint256 public eventCounter;
    
    // ========== EVENTS ==========
    
    event AdminSet(address indexed admin);
    event HumanAdded(address indexed humanAddress, string ipfsHash);
    event HumanValidationUpdated(address indexed humanAddress, bool validated);
    event HumanImageUpdated(address indexed humanAddress, string newIpfsHash);
    event EventCreated(uint256 indexed eventId, string name, uint256 participantCount);
    event FundsDistributed(uint256 indexed eventId, address indexed token, uint256 totalAmount, uint256 recipientCount);
    
    // ========== ERRORS ==========
    
    error HumanAlreadyExists();
    error HumanNotFound();
    error EventNotFound();
    error InvalidAddress();
    error InvalidAmount();
    error NoValidatedParticipants();
    error TransferFailed();
    
    // ========== CONSTRUCTOR ==========
    
    constructor() Ownable(msg.sender) {
        emit AdminSet(msg.sender);
    }
    
    // ========== HUMAN MANAGEMENT ==========
    
    /**
     * @notice Add a new human to the registry
     * @param _address Human's wallet address
     * @param _ipfsHash IPFS hash of the human's image
     */
    function addHuman(address _address, string calldata _ipfsHash) external onlyOwner {
        if (_address == address(0)) revert InvalidAddress();
        if (humans[_address].exists) revert HumanAlreadyExists();
        
        humans[_address] = Human({
            walletAddress: _address,
            ipfsHash: _ipfsHash,
            validated: false,
            exists: true
        });
        
        humanAddresses.push(_address);
        
        emit HumanAdded(_address, _ipfsHash);
    }
    
    /**
     * @notice Update human validation status
     * @param _address Human's wallet address
     * @param _validated New validation status
     */
    function updateHumanValidation(address _address, bool _validated) external onlyOwner {
        if (!humans[_address].exists) revert HumanNotFound();
        
        humans[_address].validated = _validated;
        
        emit HumanValidationUpdated(_address, _validated);
    }
    
    /**
     * @notice Update human's IPFS image hash
     * @param _address Human's wallet address
     * @param _ipfsHash New IPFS hash
     */
    function updateHumanImage(address _address, string calldata _ipfsHash) external onlyOwner {
        if (!humans[_address].exists) revert HumanNotFound();
        
        humans[_address].ipfsHash = _ipfsHash;
        
        emit HumanImageUpdated(_address, _ipfsHash);
    }
    
    /**
     * @notice Get human information
     * @param _address Human's wallet address
     * @return Human struct
     */
    function getHuman(address _address) external view returns (Human memory) {
        if (!humans[_address].exists) revert HumanNotFound();
        return humans[_address];
    }
    
    /**
     * @notice Get all humans (paginated)
     * @param _start Start index
     * @param _limit Number of results
     * @return Array of Human structs
     */
    function getAllHumans(uint256 _start, uint256 _limit) external view returns (Human[] memory) {
        uint256 end = _start + _limit;
        if (end > humanAddresses.length) {
            end = humanAddresses.length;
        }
        
        uint256 resultLength = end - _start;
        Human[] memory result = new Human[](resultLength);
        
        for (uint256 i = 0; i < resultLength; i++) {
            address addr = humanAddresses[_start + i];
            result[i] = humans[addr];
        }
        
        return result;
    }
    
    /**
     * @notice Get all validated humans
     * @return Array of validated addresses
     */
    function getValidatedHumans() external view returns (address[] memory) {
        uint256 validatedCount = 0;
        
        // Count validated humans
        for (uint256 i = 0; i < humanAddresses.length; i++) {
            if (humans[humanAddresses[i]].validated) {
                validatedCount++;
            }
        }
        
        // Build result array
        address[] memory result = new address[](validatedCount);
        uint256 resultIndex = 0;
        
        for (uint256 i = 0; i < humanAddresses.length; i++) {
            if (humans[humanAddresses[i]].validated) {
                result[resultIndex] = humanAddresses[i];
                resultIndex++;
            }
        }
        
        return result;
    }
    
    /**
     * @notice Get total number of humans
     * @return Total count
     */
    function getTotalHumans() external view returns (uint256) {
        return humanAddresses.length;
    }
    
    // ========== EVENT MANAGEMENT ==========
    
    /**
     * @notice Create a new event with participants
     * @param _name Event name
     * @param _participants Array of participant addresses
     * @return Event ID
     */
    function createEvent(string calldata _name, address[] calldata _participants) external onlyOwner returns (uint256) {
        uint256 eventId = eventCounter++;
        
        events[eventId] = Event({
            id: eventId,
            name: _name,
            participants: _participants,
            exists: true
        });
        
        emit EventCreated(eventId, _name, _participants.length);
        
        return eventId;
    }
    
    /**
     * @notice Get event information
     * @param _eventId Event ID
     * @return Event struct
     */
    function getEvent(uint256 _eventId) external view returns (Event memory) {
        if (!events[_eventId].exists) revert EventNotFound();
        return events[_eventId];
    }
    
    /**
     * @notice Get validated participants for an event
     * @param _eventId Event ID
     * @return Array of validated participant addresses
     */
    function getValidatedParticipants(uint256 _eventId) public view returns (address[] memory) {
        if (!events[_eventId].exists) revert EventNotFound();
        
        Event memory evt = events[_eventId];
        uint256 validatedCount = 0;
        
        // Count validated participants
        for (uint256 i = 0; i < evt.participants.length; i++) {
            if (humans[evt.participants[i]].validated) {
                validatedCount++;
            }
        }
        
        if (validatedCount == 0) revert NoValidatedParticipants();
        
        // Build result array
        address[] memory result = new address[](validatedCount);
        uint256 resultIndex = 0;
        
        for (uint256 i = 0; i < evt.participants.length; i++) {
            if (humans[evt.participants[i]].validated) {
                result[resultIndex] = evt.participants[i];
                resultIndex++;
            }
        }
        
        return result;
    }
    
    // ========== DISTRIBUTION ==========
    
    /**
     * @notice Distribute tokens to validated participants of an event
     * @param _eventId Event ID
     * @param _token Token address (address(0) for native CELO)
     * @param _totalAmount Total amount to distribute
     */
    function distributeToEvent(
        uint256 _eventId,
        address _token,
        uint256 _totalAmount
    ) external payable onlyOwner nonReentrant {
        if (_totalAmount == 0) revert InvalidAmount();
        
        address[] memory validatedParticipants = getValidatedParticipants(_eventId);
        uint256 amountPerRecipient = _totalAmount / validatedParticipants.length;
        
        if (_token == address(0)) {
            // Distribute native CELO
            for (uint256 i = 0; i < validatedParticipants.length; i++) {
                (bool success, ) = validatedParticipants[i].call{value: amountPerRecipient}("");
                if (!success) revert TransferFailed();
            }
        } else {
            // Distribute ERC20 token
            IERC20 token = IERC20(_token);
            
            for (uint256 i = 0; i < validatedParticipants.length; i++) {
                bool success = token.transferFrom(msg.sender, validatedParticipants[i], amountPerRecipient);
                if (!success) revert TransferFailed();
            }
        }
        
        emit FundsDistributed(_eventId, _token, _totalAmount, validatedParticipants.length);
    }
    
    /**
     * @notice Distribute tokens to specific addresses
     * @param _recipients Array of recipient addresses
     * @param _token Token address (address(0) for native CELO)
     * @param _totalAmount Total amount to distribute
     */
    function distributeToAddresses(
        address[] calldata _recipients,
        address _token,
        uint256 _totalAmount
    ) external payable onlyOwner nonReentrant {
        if (_recipients.length == 0) revert NoValidatedParticipants();
        if (_totalAmount == 0) revert InvalidAmount();
        
        uint256 amountPerRecipient = _totalAmount / _recipients.length;
        
        if (_token == address(0)) {
            // Distribute native CELO
            for (uint256 i = 0; i < _recipients.length; i++) {
                (bool success, ) = _recipients[i].call{value: amountPerRecipient}("");
                if (!success) revert TransferFailed();
            }
        } else {
            // Distribute ERC20 token
            IERC20 token = IERC20(_token);
            
            for (uint256 i = 0; i < _recipients.length; i++) {
                bool success = token.transferFrom(msg.sender, _recipients[i], amountPerRecipient);
                if (!success) revert TransferFailed();
            }
        }
        
        emit FundsDistributed(0, _token, _totalAmount, _recipients.length);
    }
    
    // ========== FALLBACK ==========
    
    receive() external payable {}
}
