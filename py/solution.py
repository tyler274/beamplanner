from typing import Dict, List, Tuple
from util import Color, Sat, User, Vector3
import math
import random

# Degrees
MINIMUM_BEAM_ANGLE = 10
MAX_ALLOWABLE_BEAM_ANGLE = 45
MAX_ALLOWED_USERS = 32
MAX_COLOR_OPTIONS = 4

random.seed()

def get_index_shifts(users: Dict[User, Vector3], sats: Dict[Sat, Vector3]) -> Tuple[int, int]:
    """Get index shifts for satellite and users data."""
    index_shift_sats = int(list(sats.keys())[0])    
    index_shift_users = int(list(users.keys())[0])
    return index_shift_sats, index_shift_users

def color_next(color):
    return (color % MAX_COLOR_OPTIONS) + 1

def sum_rows(valid_connections, rows, cols):
    """
    Calculates the sum of valid connections in each row of a matrix.
    Args:
        valid_connections (list): A 2D matrix representing the valid connections.
        rows (int): The number of rows in the matrix.
        cols (int): The number of columns in the matrix.
    Returns:
        list: A list containing the sum of valid connections in each row.
    """

    return [sum(1 for c in range(cols) if valid_connections[r][c] >= 1) for r in range(rows)]

def sum_cols(valid_connections, rows, cols):
    """
    Calculates the sum of valid connections for each column in a matrix.
    Args:
        valid_connections (list): A 2D matrix representing the valid connections.
        rows (int): The number of rows in the matrix.
        cols (int): The number of columns in the matrix.
    Returns:
        list: A list containing the sum of valid connections for each column.
    """

    return [sum(1 for r in range(rows) if valid_connections[r][c] >= 1) for c in range(cols)]

def initialize_valid_connections(n_sats: int, n_users: int) -> List[List[int]]:
    """Initialize valid connections matrix."""
    return [[0] * n_users for _ in range(n_sats)]

def calculate_beam_angle(user: Vector3, sat: Vector3) -> float:
    """Calculate the beam angle between a user and a satellite."""
    center = Vector3(0, 0, 0)
    return 180 - user.angle_between(center, sat)
4
def sort_values(values, n_values, index_shift) -> List[Tuple[int, float, Vector3]]:
    """Normalize Users and Sats and then sort by dot product."""
    # Find the largest x, y, and z values for normalization
    largest_x = max(abs(values[i + index_shift].x) for i in range(n_values))
    largest_y = max(abs(values[i + index_shift].y) for i in range(n_values))
    largest_z = max(abs(values[i + index_shift].z) for i in range(n_values))

    # Add a small epsilon to avoid division by zero
    epsilon = 1e-10
    scaling_vector = Vector3(largest_x + epsilon, largest_y + epsilon, largest_z + epsilon)
    basis_vector = Vector3(1, 1, 1)

    sorted_values = []
    for i in range(n_values):
        value = values[i + index_shift]
        normalized_value = Vector3(value.x / scaling_vector.x, value.y / scaling_vector.y, value.z / scaling_vector.z)
        dot_prod = normalized_value.dot(basis_vector)
        sorted_values.append((i + index_shift, dot_prod, value))

    # Sort by the dot product in descending order
    sorted_values.sort(key=lambda x: x[1], reverse=True)
    return sorted_values

def get_sorted_values(users: Dict[User, Vector3], sats: Dict[Sat, Vector3], index_shift_users: int, index_shift_sats: int) -> Tuple[List, List]:
    """Sort and normalize users and satellites."""
    n_users = len(users)
    n_sats = len(sats)
    users_sorted = sort_values(users, n_users, index_shift_users)
    satellites_sorted = sort_values(sats, n_sats, index_shift_sats)
    return users_sorted, satellites_sorted


def validate_sat_congestion(sats, users, valid_connections, sats_sorted, users_sorted, index_shift_sats, index_shift_users, shift_colors):
    """(Pre)calculate the possible congestion between satellites and users, for valid configurations gemoetry-wise."""
    for sat_tuple in sats_sorted:
        sat_index = sat_tuple[0] - index_shift_sats
        sat_vector = sats[sat_tuple[0]]

        # Construct a list with users that have valid connections to this satelite
        users_with_valid_connections = [(user_tuple[0], user_tuple[1]) for user_tuple in users_sorted if valid_connections[sat_index][user_tuple[0] - index_shift_users] >= 1]

        # Check each user against each other user pairwise to check if they interfere
        for i, first_user_tuple in enumerate(users_with_valid_connections):
            first_user_index = first_user_tuple[0] - index_shift_users
            first_user_vector = users[first_user_tuple[0]]

            for j in range(i + 1, len(users_with_valid_connections)):
                second_user_index = users_with_valid_connections[j][0] - index_shift_users
                second_user_vector = users[users_with_valid_connections[j][0]]

                # Find the angle between the satellite and the two users
                sat_user_angle = sat_vector.angle_between(first_user_vector, second_user_vector)

                # If the angle is larger than the minimum needed, check if the users have the same color
                if sat_user_angle < MINIMUM_BEAM_ANGLE:
                    first_user_color = valid_connections[sat_index][first_user_index]
                    second_user_color = valid_connections[sat_index][second_user_index]

                    # If the users have the same color, remove the connection or shift the color
                    if first_user_color == second_user_color:
                        # mix up colors if needed
                        if shift_colors:
                            color = color_next(second_user_color)
                            valid_connections[sat_index][second_user_index] = color
                        else:
                            valid_connections[sat_index][second_user_index] = 0

def remove_excess_satelites_per_user(valid_connections, sorted_satellites, sorted_users, index_shift_sats):
    """Remove users from satellites that have more than the allowed number of users."""
    col_totals = sum_cols(valid_connections, len(sorted_satellites), len(sorted_users))

    # Remove extraneous satelites per user, if more than one satelite is assigned
    for user in range(len(sorted_users)):
        user_sats = col_totals[user]

        # User has extra sats
        if user_sats > 1:
            # Construct a list of all valid satelites for said user
            valid_satelites = []
            for satelite_tuple in sorted_satellites:
                index_satelite = satelite_tuple[0] - index_shift_sats
                # If the beam is valid, append a Tuple[satelite index, dot product] to the list
                if valid_connections[index_satelite][user] >= 1:
                    valid_satelites.append((satelite_tuple[0], satelite_tuple[1])) 

            # Shuffle valid_satelites and select the first satelite, and the color for it. 
            valid_satelites = sorted(valid_satelites, key=lambda x: random.random())
            index_chosen = valid_satelites[0][0] - index_shift_sats
            color = valid_connections[index_chosen][user]

            # Remove all connections 
            for satelite_tuple in valid_satelites:
                index_satelite = satelite_tuple[0] - index_shift_sats
                valid_connections[index_satelite][user] = 0

            # Add back chosen connection
            valid_connections[index_chosen][user] = color

def remove_excess_users_per_satellite(valid_connections, sorted_sats, sorted_users):
    """Remove excess users per satellite."""
    row_totals = sum_rows(valid_connections, len(sorted_sats), len(sorted_users))

    for s in range(len(sorted_sats)):
        sat_users = row_totals[s]

        # Satelite has more than 1 valid user
        if sat_users > MAX_ALLOWED_USERS:
            users_allocated = 0

            # Searching users for valid connections
            for u in range(len(sorted_users)):

                # Found a valid connection
                if valid_connections[s][u] >= 1:

                    # Note the first connection found
                    if users_allocated < MAX_ALLOWED_USERS:
                        users_allocated += 1
                    
                    # Already have max users allocated
                    else:
                        # Remove connection
                        valid_connections[s][u] = 0

def format_solution(valid_connections, sats, users, index_shift_sats, index_shift_users):
    """Format the solution dictionary."""
    solution = {}
    for u in range(len(users)):
        for s in range(len(sats)):
            if valid_connections[s][u] >= 1:
                color = valid_connections[s][u]
                solution[u + index_shift_users] = (s + index_shift_sats, color)
    return solution

def initialize_colors(users, sats, users_sorted, sats_sorted, index_shift_users, index_shift_sats, valid_connections):
    """Initialize colors for the valid connections matrix."""

    # Initially its 'A'
    color = 1

    for user_tuple in users_sorted:
        user = users[user_tuple[0]]

        for sat_tuple in sats_sorted:
            sat = sats[sat_tuple[0]]
            center = Vector3(0,0,0)
            beam_angle = calculate_beam_angle(user, sat)
            # Check if the beam angle is within the acceptable range
            if beam_angle < MAX_ALLOWABLE_BEAM_ANGLE:
                satellite_index = sat_tuple[0] - index_shift_sats
                user_index = user_tuple[0] - index_shift_users
                valid_connections[satellite_index][user_index] = color_next(color)
    

def solve(users: Dict[User, Vector3], sats: Dict[Sat, Vector3]) -> Dict[User, Tuple[Sat, Color]]:
    """Assign users to satellites respecting all constraints."""
    solution = {}

    # Get indexes for users and sats
    index_shift_sats, index_shift_users = get_index_shifts(users, sats)

    # Get sorted values
    users_sorted, sats_sorted = get_sorted_values(users, sats, index_shift_users, index_shift_sats)

    # Initialize valid connections matrix
    valid_connections = initialize_valid_connections(len(sats), len(users))

    # Initialize colors
    initialize_colors(users, sats, users_sorted, sats_sorted, index_shift_users, index_shift_sats, valid_connections)
    
    # Remove excess users from satellites
    remove_excess_satelites_per_user(valid_connections, sats_sorted, users_sorted, index_shift_sats)
            
    # Check beams from each (sorted) satellite for congestion
    iterations = 2
    shift_color = True # Rotate color 
    for i in range(iterations):
        validate_sat_congestion(sats, users, valid_connections, sats_sorted, users_sorted, index_shift_sats, index_shift_users, shift_color)
    
    shift_color = False
    validate_sat_congestion(sats, users, valid_connections, sats_sorted, users_sorted, index_shift_sats, index_shift_users, shift_color)

    remove_excess_users_per_satellite(valid_connections, sats_sorted, users_sorted)

    # Format the solution
    solution = format_solution(valid_connections, sats, users, index_shift_sats, index_shift_users)

    return solution